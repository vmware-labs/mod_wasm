<?php

use Twig\Environment;
use Twig\Error\LoaderError;
use Twig\Error\RuntimeError;
use Twig\Extension\SandboxExtension;
use Twig\Markup;
use Twig\Sandbox\SecurityError;
use Twig\Sandbox\SecurityNotAllowedTagError;
use Twig\Sandbox\SecurityNotAllowedFilterError;
use Twig\Sandbox\SecurityNotAllowedFunctionError;
use Twig\Source;
use Twig\Template;

/* @claro/entity-add-list.html.twig */
class __TwigTemplate_97a5c9ea565dd8e4a0394a85dabd7ff0 extends Template
{
    private $source;
    private $macros = [];

    public function __construct(Environment $env)
    {
        parent::__construct($env);

        $this->source = $this->getSourceContext();

        $this->parent = false;

        $this->blocks = [
        ];
        $this->sandbox = $this->env->getExtension('\Twig\Extension\SandboxExtension');
        $this->checkSecurity();
    }

    protected function doDisplay(array $context, array $blocks = [])
    {
        $macros = $this->macros;
        // line 19
        $context["item_classes"] = [0 => "admin-item"];
        // line 23
        if ( !twig_test_empty(($context["bundles"] ?? null))) {
            // line 24
            echo "  <dl";
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, ($context["attributes"] ?? null), "addClass", [0 => "admin-list"], "method", false, false, true, 24), 24, $this->source), "html", null, true);
            echo ">
    ";
            // line 25
            $context['_parent'] = $context;
            $context['_seq'] = twig_ensure_traversable(($context["bundles"] ?? null));
            foreach ($context['_seq'] as $context["_key"] => $context["bundle"]) {
                // line 26
                echo "      ";
                // line 30
                echo "      ";
                $context["bundle_attributes"] = ((twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, $context["bundle"], "add_link", [], "any", false, false, true, 30), "url", [], "any", false, false, true, 30), "getOption", [0 => "attributes"], "method", false, false, true, 30)) ? (twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, $context["bundle"], "add_link", [], "any", false, false, true, 30), "url", [], "any", false, false, true, 30), "getOption", [0 => "attributes"], "method", false, false, true, 30)) : ([]));
                // line 31
                echo "      ";
                $context["link_attributes"] = twig_get_attribute($this->env, $this->source, $this->extensions['Drupal\Core\Template\TwigExtension']->createAttribute($this->sandbox->ensureToStringAllowed(($context["bundle_attributes"] ?? null), 31, $this->source)), "addClass", [0 => "admin-item__link"], "method", false, false, true, 31);
                // line 32
                echo "      <div";
                echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->extensions['Drupal\Core\Template\TwigExtension']->createAttribute(["class" => ($context["item_classes"] ?? null)]), "html", null, true);
                echo ">
        <dt class=\"admin-item__title\">
          <a href=\"";
                // line 34
                echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, $context["bundle"], "add_link", [], "any", false, false, true, 34), "url", [], "any", false, false, true, 34), 34, $this->source), "html", null, true);
                echo "\"";
                echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->extensions['Drupal\Core\Template\TwigExtension']->withoutFilter($this->sandbox->ensureToStringAllowed(($context["link_attributes"] ?? null), 34, $this->source), "href"), "html", null, true);
                echo ">
            ";
                // line 35
                echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, $context["bundle"], "add_link", [], "any", false, false, true, 35), "text", [], "any", false, false, true, 35), 35, $this->source), "html", null, true);
                echo "
          </a>
        </dt>
        ";
                // line 39
                echo "        ";
                if (twig_get_attribute($this->env, $this->source, $context["bundle"], "description", [], "any", false, false, true, 39)) {
                    // line 40
                    echo "          <dd class=\"admin-item__description\">";
                    echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, $context["bundle"], "description", [], "any", false, false, true, 40), 40, $this->source), "html", null, true);
                    echo "</dd>
        ";
                }
                // line 42
                echo "      </div>
    ";
            }
            $_parent = $context['_parent'];
            unset($context['_seq'], $context['_iterated'], $context['_key'], $context['bundle'], $context['_parent'], $context['loop']);
            $context = array_intersect_key($context, $_parent) + $_parent;
            // line 44
            echo "  </dl>
";
        } elseif ( !twig_test_empty(        // line 45
($context["add_bundle_message"] ?? null))) {
            // line 46
            echo "  <p>
    ";
            // line 47
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["add_bundle_message"] ?? null), 47, $this->source), "html", null, true);
            echo "
  </p>
";
        }
    }

    public function getTemplateName()
    {
        return "@claro/entity-add-list.html.twig";
    }

    public function isTraitable()
    {
        return false;
    }

    public function getDebugInfo()
    {
        return array (  102 => 47,  99 => 46,  97 => 45,  94 => 44,  87 => 42,  81 => 40,  78 => 39,  72 => 35,  66 => 34,  60 => 32,  57 => 31,  54 => 30,  52 => 26,  48 => 25,  43 => 24,  41 => 23,  39 => 19,);
    }

    public function getSourceContext()
    {
        return new Source("", "@claro/entity-add-list.html.twig", "/usr/local/apache2/htdocs/drupal-10-zero/core/themes/claro/templates/entity-add-list.html.twig");
    }
    
    public function checkSecurity()
    {
        static $tags = array("set" => 19, "if" => 23, "for" => 25);
        static $filters = array("escape" => 24, "without" => 34);
        static $functions = array("create_attribute" => 31);

        try {
            $this->sandbox->checkSecurity(
                ['set', 'if', 'for'],
                ['escape', 'without'],
                ['create_attribute']
            );
        } catch (SecurityError $e) {
            $e->setSourceContext($this->source);

            if ($e instanceof SecurityNotAllowedTagError && isset($tags[$e->getTagName()])) {
                $e->setTemplateLine($tags[$e->getTagName()]);
            } elseif ($e instanceof SecurityNotAllowedFilterError && isset($filters[$e->getFilterName()])) {
                $e->setTemplateLine($filters[$e->getFilterName()]);
            } elseif ($e instanceof SecurityNotAllowedFunctionError && isset($functions[$e->getFunctionName()])) {
                $e->setTemplateLine($functions[$e->getFunctionName()]);
            }

            throw $e;
        }

    }
}
